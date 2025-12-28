macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! MAX_REFCOUNT {
    () => {
        deps!();
        # [doc = " A soft limit on the amount of references that may be made to an `Arc`."] # [doc = ""] # [doc = " Going above this limit will abort your program (although not"] # [doc = " necessarily) at _exactly_ `MAX_REFCOUNT + 1` references."] const MAX_REFCOUNT : usize = (isize :: MAX) as usize ;
    };
}

MAX_REFCOUNT!();