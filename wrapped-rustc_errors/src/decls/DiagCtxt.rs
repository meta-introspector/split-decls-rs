macro_rules! deps {
    () => {
        DiagCtxtInner!();
    };
}

macro_rules! DiagCtxt {
    () => {
        deps!();
        # [doc = " A `DiagCtxt` deals with errors and other compiler output."] # [doc = " Certain errors (fatal, bug, unimpl) may cause immediate exit,"] # [doc = " others log errors for later reporting."] pub struct DiagCtxt { inner : Lock < DiagCtxtInner > , }
    };
}

DiagCtxt!()