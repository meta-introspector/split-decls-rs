macro_rules! deps {
    () => {
        PlaceValue!();
        OperandValue!();
        BuilderMethods!();
        PlaceRef!();
    };
}

macro_rules! impl_509 {
    () => {
        deps!();
        impl < V : CodegenObject > PlaceValue < V > { # [doc = " Constructor for the ordinary case of `Sized` types."] # [doc = ""] # [doc = " Sets `llextra` to `None`."] pub fn new_sized (llval : V , align : Align) -> PlaceValue < V > { PlaceValue { llval , llextra : None , align } } # [doc = " Allocates a stack slot in the function for a value"] # [doc = " of the specified size and alignment."] # [doc = ""] # [doc = " The allocation itself is untyped."] pub fn alloca < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx , Value = V > > (bx : & mut Bx , size : Size , align : Align ,) -> PlaceValue < V > { let llval = bx . alloca (size , align) ; PlaceValue :: new_sized (llval , align) } # [doc = " Creates a `PlaceRef` to this location with the given type."] pub fn with_type < 'tcx > (self , layout : TyAndLayout < 'tcx >) -> PlaceRef < 'tcx , V > { assert ! (layout . is_unsized () || layout . is_uninhabited () || self . llextra . is_none () , "Had pointer metadata {:?} for sized type {layout:?}" , self . llextra ,) ; PlaceRef { val : self , layout } } # [doc = " Gets the pointer to this place as an [`OperandValue::Immediate`]"] # [doc = " or, for those needing metadata, an [`OperandValue::Pair`]."] # [doc = ""] # [doc = " This is the inverse of [`OperandValue::deref`]."] pub fn address (self) -> OperandValue < V > { if let Some (llextra) = self . llextra { OperandValue :: Pair (self . llval , llextra) } else { OperandValue :: Immediate (self . llval) } } }
    };
}

impl_509!()