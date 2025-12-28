macro_rules! DlError {
    () => {
        # [doc = " A `dlerror` error."] pub struct DlError (pub (crate) CString) ;
    };
}

DlError!()