macro_rules! deps {
    () => {
        Builder!();
        FormatFn!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Builder { # [doc = " Convert the format into a callable function."] # [doc = ""] # [doc = " If the `custom_format` is `Some`, then any `default_format` switches are ignored."] # [doc = " If the `custom_format` is `None`, then a default format is returned."] # [doc = " Any `default_format` switches set to `false` won't be written by the format."] pub (crate) fn build (& mut self) -> FormatFn { assert ! (! self . built , "attempt to re-use consumed builder") ; let built = mem :: replace (self , Builder { built : true , .. Default :: default () } ,) ; if let Some (fmt) = built . custom_format { fmt } else { Box :: new (built . default_format) } } }
    };
}

impl_70!()