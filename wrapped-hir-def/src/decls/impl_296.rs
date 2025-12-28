macro_rules! deps {
    () => {
        ExpressionStore!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl Index < TypeRefId > for ExpressionStore { type Output = TypeRef ; # [inline] fn index (& self , b : TypeRefId) -> & TypeRef { & self . types [b] } }
    };
}

impl_296!()