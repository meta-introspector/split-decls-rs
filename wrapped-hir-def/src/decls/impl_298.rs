macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl Index < PathId > for ExpressionStore { type Output = Path ; # [inline] fn index (& self , index : PathId) -> & Self :: Output { let TypeRef :: Path (path) = & self [index . type_ref ()] else { unreachable ! ("`PathId` always points to `TypeRef::Path`") ; } ; path } }
    };
}

impl_298!()