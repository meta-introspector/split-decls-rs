macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! assume_scalar_range {
    () => {
        deps!();
        # [doc = " Emits an `assume` call that `imm`'s value is within the known range of `scalar`."] # [doc = ""] # [doc = " If `known` is `Some`, only emits the assume if it's more specific than"] # [doc = " whatever is already known from the range of *that* scalar."] fn assume_scalar_range < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , imm : Bx :: Value , scalar : abi :: Scalar , backend_ty : Bx :: Type , known : Option < & abi :: Scalar > ,) { if matches ! (bx . cx () . sess () . opts . optimize , OptLevel :: No) { return ; } match (scalar , known) { (abi :: Scalar :: Union { .. } , _) => return , (_ , None) => { if scalar . is_always_valid (bx . cx ()) { return ; } } (abi :: Scalar :: Initialized { valid_range , .. } , Some (known)) => { let known_range = known . valid_range (bx . cx ()) ; if valid_range . contains_range (known_range , scalar . size (bx . cx ())) { return ; } } } match scalar . primitive () { abi :: Primitive :: Int (..) => { let range = scalar . valid_range (bx . cx ()) ; bx . assume_integer_range (imm , backend_ty , range) ; } abi :: Primitive :: Pointer (abi :: AddressSpace :: ZERO) if ! scalar . valid_range (bx . cx ()) . contains (0) => { bx . assume_nonnull (imm) ; } abi :: Primitive :: Pointer (..) | abi :: Primitive :: Float (..) => { } } }
    };
}

assume_scalar_range!();