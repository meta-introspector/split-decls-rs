macro_rules! macro_83 {
    () => {
        bitflags ! { # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub struct StructFlags : u8 { # [doc = " Indicates whether the struct has a `#[rustc_has_incoherent_inherent_impls]` attribute."] const RUSTC_HAS_INCOHERENT_INHERENT_IMPLS = 1 << 1 ; # [doc = " Indicates whether the struct has a `#[fundamental]` attribute."] const FUNDAMENTAL = 1 << 2 ; # [doc = " Indicates whether the struct is `PhantomData`."] const IS_PHANTOM_DATA = 1 << 3 ; # [doc = " Indicates whether this struct is `Box`."] const IS_BOX = 1 << 4 ; # [doc = " Indicates whether this struct is `ManuallyDrop`."] const IS_MANUALLY_DROP = 1 << 5 ; # [doc = " Indicates whether this struct is `UnsafeCell`."] const IS_UNSAFE_CELL = 1 << 6 ; # [doc = " Indicates whether this struct is `UnsafePinned`."] const IS_UNSAFE_PINNED = 1 << 7 ; } }
    };
}

macro_83!()