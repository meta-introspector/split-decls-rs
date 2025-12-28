macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! IInspectable {
    () => {
        deps!();
        # [doc = " Parent interface for all WinRT interfaces."] # [doc = ""] # [doc = " A WinRT object that may be used as a polymorphic stand-in for any WinRT class, interface, or boxed value."] # [doc = " [`IInspectable`] represents the"] # [doc = " [IInspectable](https://docs.microsoft.com/en-us/windows/win32/api/inspectable/nn-inspectable-iinspectable)"] # [doc = " interface."] # [repr (transparent)] # [derive (Clone , PartialEq , Eq , Debug)] pub struct IInspectable (pub IUnknown) ;
    };
}

IInspectable!();