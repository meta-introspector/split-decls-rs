// rustc_abi module - Application Binary Interface types and utilities

pub mod layout {
    pub struct Layout;
    pub struct LayoutError;
    pub struct TyAndLayout<T> {
        pub ty: T,
        pub layout: Layout,
    }
}

pub mod size {
    pub struct Size(pub u64);
    
    impl Size {
        pub const ZERO: Size = Size(0);
        pub fn bytes(self) -> u64 { self.0 }
    }
}

pub mod align {
    pub struct Align(pub u64);
    
    impl Align {
        pub const ONE: Align = Align(1);
        pub fn bytes(self) -> u64 { self.0 }
    }
}

pub use layout::*;
pub use size::*;
pub use align::*;

// Common ABI-related types
pub struct FieldsShape;
pub struct Variants;
pub struct Scalar;
pub struct Primitive;
