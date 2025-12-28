macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! with_dlerror {
    () => {
        deps!();
        # [doc = " Run code and handle errors reported by `dlerror`."] # [doc = ""] # [doc = " This function first executes the `closure` function containing calls to the functions that"] # [doc = " report their errors via `dlerror`. This closure may return either `None` or `Some(*)` to"] # [doc = " further affect operation of this function."] # [doc = ""] # [doc = " In case the `closure` returns `None`, `with_dlerror` inspects the `dlerror`. `dlerror` may"] # [doc = " decide to not provide any error description, in which case `Err(None)` is returned to the"] # [doc = " caller. Otherwise the `error` callback is invoked to allow inspection and conversion of the"] # [doc = " error message. The conversion result is returned as `Err(Some(Error))`."] # [doc = ""] # [doc = " If the operations that report their errors via `dlerror` were all successful, `closure` should"] # [doc = " return `Some(T)` instead. In this case `dlerror` is not inspected at all."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " The whole `dlerror` handling scheme is done via setting and querying some global state. For"] # [doc = " that reason it is not safe to use dynamic library loading in MT-capable environment at all."] # [doc = " Only in POSIX 2008+TC1 a thread-local state was allowed for `dlerror`, making the dl* family of"] # [doc = " functions possibly MT-safe, depending on the implementation of `dlerror`."] # [doc = ""] # [doc = " In practice (as of 2020-04-01) most of the widely used targets use a thread-local for error"] # [doc = " state and have been doing so for a long time."] pub fn with_dlerror < T , F , Error > (closure : F , error : fn (& CStr) -> Error) -> Result < T , Option < Error > > where F : FnOnce () -> Option < T > , { closure () . ok_or_else (| | unsafe { let dlerror_str = dlerror () ; if dlerror_str . is_null () { None } else { Some (error (CStr :: from_ptr (dlerror_str))) } }) }
    };
}

with_dlerror!()