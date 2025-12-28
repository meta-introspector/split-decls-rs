macro_rules! deps {
    () => {
        NTSTATUS!();
        HRESULT!();
        Result!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl NTSTATUS { # [doc = " Returns [`true`] if `self` is a success code."] # [inline] pub const fn is_ok (self) -> bool { self . 0 >= 0 } # [doc = " Returns [`true`] if `self` is a failure code."] # [inline] pub const fn is_err (self) -> bool { ! self . is_ok () } # [doc = " Maps an NT error code to an HRESULT value."] # [inline] pub const fn to_hresult (self) -> HRESULT { HRESULT (if self . 0 >= 0 { self . 0 } else { self . 0 | 0x1000_0000 }) } # [doc = " Asserts that `self` is a success code."] # [doc = ""] # [doc = " This will invoke the [`panic!`] macro if `self` is a failure code and display"] # [doc = " the [`NTSTATUS`] value for diagnostics."] # [inline] # [track_caller] pub fn unwrap (self) { assert ! (self . is_ok () , "NTSTATUS 0x{:X}" , self . 0) ; } # [doc = " Converts the [`NTSTATUS`] to [`Result<()>`][Result<_>]."] # [inline] pub fn ok (self) -> Result < () > { self . to_hresult () . ok () } }
    };
}

impl_106!()