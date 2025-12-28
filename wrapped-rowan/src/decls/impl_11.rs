macro_rules! deps {
    () => {
        NodeData!();
        Elem!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        unsafe impl sll :: Elem for NodeData { fn prev (& self) -> & Cell < * const Self > { & self . prev } fn next (& self) -> & Cell < * const Self > { & self . next } fn key (& self) -> & Cell < u32 > { & self . index } }
    };
}

impl_11!()