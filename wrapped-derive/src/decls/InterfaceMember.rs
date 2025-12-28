macro_rules! InterfaceMember {
    () => {
        # [derive (FromVariant)] pub struct InterfaceMember { pub ident : Ident , pub fields : Fields < syn :: Type > , }
    };
}

InterfaceMember!();