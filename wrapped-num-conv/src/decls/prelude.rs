macro_rules! deps {
    () => {
        Truncate!();
        Extend!();
    };
}

macro_rules! prelude {
    () => {
        deps!();
        # [doc = " Anonymously import all extension traits."] # [doc = ""] # [doc = " This allows you to use the methods without worrying about polluting the namespace or importing"] # [doc = " them individually."] # [doc = ""] # [doc = " ```rust"] # [doc = " use num_conv::prelude::*;"] # [doc = " ```"] pub mod prelude { pub use crate :: { Extend as _ , Truncate as _ } ; }
    };
}

prelude!()