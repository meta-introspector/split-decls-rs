macro_rules! deps {
    () => {
        Var!();
        Env!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < 'a > Env < 'a > { # [doc = " Get a default set of environment variables."] pub fn new () -> Self { Self :: default () } # [doc = " Specify an environment variable to read the filter from."] pub fn filter < E > (mut self , filter_env : E) -> Self where E : Into < Cow < 'a , str > > , { self . filter = Var :: new (filter_env) ; self } # [doc = " Specify an environment variable to read the filter from."] # [doc = ""] # [doc = " If the variable is not set, the default value will be used."] pub fn filter_or < E , V > (mut self , filter_env : E , default : V) -> Self where E : Into < Cow < 'a , str > > , V : Into < Cow < 'a , str > > , { self . filter = Var :: new_with_default (filter_env , default) ; self } # [doc = " Use the default environment variable to read the filter from."] # [doc = ""] # [doc = " If the variable is not set, the default value will be used."] pub fn default_filter_or < V > (mut self , default : V) -> Self where V : Into < Cow < 'a , str > > , { self . filter = Var :: new_with_default (DEFAULT_FILTER_ENV , default) ; self } fn get_filter (& self) -> Option < String > { self . filter . get () } # [doc = " Specify an environment variable to read the style from."] pub fn write_style < E > (mut self , write_style_env : E) -> Self where E : Into < Cow < 'a , str > > , { self . write_style = Var :: new (write_style_env) ; self } # [doc = " Specify an environment variable to read the style from."] # [doc = ""] # [doc = " If the variable is not set, the default value will be used."] pub fn write_style_or < E , V > (mut self , write_style_env : E , default : V) -> Self where E : Into < Cow < 'a , str > > , V : Into < Cow < 'a , str > > , { self . write_style = Var :: new_with_default (write_style_env , default) ; self } # [doc = " Use the default environment variable to read the style from."] # [doc = ""] # [doc = " If the variable is not set, the default value will be used."] pub fn default_write_style_or < V > (mut self , default : V) -> Self where V : Into < Cow < 'a , str > > , { self . write_style = Var :: new_with_default (DEFAULT_WRITE_STYLE_ENV , default) ; self } fn get_write_style (& self) -> Option < String > { self . write_style . get () } }
    };
}

impl_10!();