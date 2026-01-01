pub mod crate {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct BackendRepr;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct FieldsShape;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Primitive;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Size;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAbiInterface;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TyAndLayout;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Variants;
}

pub mod reg {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Reg;
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RegKind;
}

