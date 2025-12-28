macro_rules! deps {
    () => {
        ImplementType!();
    };
}

macro_rules! InterfaceChain {
    () => {
        deps!();
        # [doc = " Describes one COM interface chain."] struct InterfaceChain { # [doc = " The name of the field for the vtable chain, e.g. `interface4_ifoo`."] field_ident : syn :: Ident , # [doc = " The name of the associated constant item for the vtable chain's initializer,"] # [doc = " e.g. `INTERFACE4_IFOO_VTABLE`."] vtable_const_ident : syn :: Ident , implement : ImplementType , }
    };
}

InterfaceChain!();