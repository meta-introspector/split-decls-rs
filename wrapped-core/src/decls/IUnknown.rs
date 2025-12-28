macro_rules! IUnknown {
    () => {
        # [doc = " Base interface for all COM interfaces."] # [doc = ""] # [doc = " All COM interfaces (and thus WinRT classes and interfaces) implement"] # [doc = " [IUnknown](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nn-unknwn-iunknown)"] # [doc = " under the hood to provide reference-counted lifetime management as well as the ability"] # [doc = " to query for additional interfaces that the object may implement."] # [repr (transparent)] pub struct IUnknown (NonNull < c_void >) ;
    };
}

IUnknown!();