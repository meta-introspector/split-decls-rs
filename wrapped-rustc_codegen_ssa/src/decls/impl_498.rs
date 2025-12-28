macro_rules! deps {
    () => {
        OperandValue!();
        PlaceValue!();
        LayoutTypeCodegenMethods!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl < V : CodegenObject > OperandValue < V > { # [doc = " Treat this value as a pointer and return the data pointer and"] # [doc = " optional metadata as backend values."] # [doc = ""] # [doc = " If you're making a place, use [`Self::deref`] instead."] pub (crate) fn pointer_parts (self) -> (V , Option < V >) { match self { OperandValue :: Immediate (llptr) => (llptr , None) , OperandValue :: Pair (llptr , llextra) => (llptr , Some (llextra)) , _ => bug ! ("OperandValue cannot be a pointer: {self:?}") , } } # [doc = " Treat this value as a pointer and return the place to which it points."] # [doc = ""] # [doc = " The pointer immediate doesn't inherently know its alignment,"] # [doc = " so you need to pass it in. If you want to get it from a type's ABI"] # [doc = " alignment, then maybe you want [`OperandRef::deref`] instead."] # [doc = ""] # [doc = " This is the inverse of [`PlaceValue::address`]."] pub (crate) fn deref (self , align : Align) -> PlaceValue < V > { let (llval , llextra) = self . pointer_parts () ; PlaceValue { llval , llextra , align } } pub (crate) fn is_expected_variant_for_type < 'tcx , Cx : LayoutTypeCodegenMethods < 'tcx > > (& self , cx : & Cx , ty : TyAndLayout < 'tcx > ,) -> bool { match self { OperandValue :: ZeroSized => ty . is_zst () , OperandValue :: Immediate (_) => cx . is_backend_immediate (ty) , OperandValue :: Pair (_ , _) => cx . is_backend_scalar_pair (ty) , OperandValue :: Ref (_) => cx . is_backend_ref (ty) , } } }
    };
}

impl_498!();