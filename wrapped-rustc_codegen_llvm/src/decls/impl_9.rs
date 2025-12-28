macro_rules! deps {
    () => {
        PassMode!();
        Builder!();
        ArgAbiExt!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'll , 'tcx > ArgAbiExt < 'll , 'tcx > for ArgAbi < 'tcx , Ty < 'tcx > > { # [doc = " Stores a direct/indirect value described by this ArgAbi into a"] # [doc = " place for the original Rust type of this argument/return."] # [doc = " Can be used for both storing formal arguments into Rust variables"] # [doc = " or results of call/invoke instructions into their destinations."] fn store (& self , bx : & mut Builder < '_ , 'll , 'tcx > , val : & 'll Value , dst : PlaceRef < 'tcx , & 'll Value > ,) { match & self . mode { PassMode :: Ignore => { } PassMode :: Indirect { attrs , meta_attrs : None , on_stack : _ } => { let align = attrs . pointee_align . unwrap_or (self . layout . align . abi) ; OperandValue :: Ref (PlaceValue :: new_sized (val , align)) . store (bx , dst) ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { bug ! ("unsized `ArgAbi` must be handled through `store_fn_arg`") ; } PassMode :: Cast { cast , pad_i32 : _ } => { let scratch_size = cast . size (bx) ; let scratch_align = cast . align (bx) ; let copy_bytes = cmp :: min (cast . unaligned_size (bx) . bytes () , self . layout . size . bytes ()) ; let llscratch = bx . alloca (scratch_size , scratch_align) ; bx . lifetime_start (llscratch , scratch_size) ; rustc_codegen_ssa :: mir :: store_cast (bx , cast , val , llscratch , scratch_align) ; bx . memcpy (dst . val . llval , self . layout . align . abi , llscratch , scratch_align , bx . const_usize (copy_bytes) , MemFlags :: empty () ,) ; bx . lifetime_end (llscratch , scratch_size) ; } _ => { OperandRef :: from_immediate_or_packed_pair (bx , val , self . layout) . val . store (bx , dst) ; } } } fn store_fn_arg (& self , bx : & mut Builder < '_ , 'll , 'tcx > , idx : & mut usize , dst : PlaceRef < 'tcx , & 'll Value > ,) { let mut next = | | { let val = llvm :: get_param (bx . llfn () , * idx as c_uint) ; * idx += 1 ; val } ; match self . mode { PassMode :: Ignore => { } PassMode :: Pair (..) => { OperandValue :: Pair (next () , next ()) . store (bx , dst) ; } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { let place_val = PlaceValue { llval : next () , llextra : Some (next ()) , align : self . layout . align . abi , } ; OperandValue :: Ref (place_val) . store (bx , dst) ; } PassMode :: Direct (_) | PassMode :: Indirect { attrs : _ , meta_attrs : None , on_stack : _ } | PassMode :: Cast { .. } => { let next_arg = next () ; self . store (bx , next_arg , dst) ; } } } }
    };
}

impl_9!()