macro_rules! deps {
    () => {
        RichFormatter!();
        ErrorKind!();
    };
}

macro_rules! KindFormatter {
    () => {
        deps!();
        # [doc = " Report [`ErrorKind`]"] # [doc = ""] # [doc = " No context is included."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **NOTE:** Consider removing the `error-context` default feature if using this to remove all"] # [doc = " overhead for [`RichFormatter`]."] # [doc = ""] # [doc = " </div>"] # [non_exhaustive] pub struct KindFormatter ;
    };
}

KindFormatter!()