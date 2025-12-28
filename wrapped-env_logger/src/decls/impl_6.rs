macro_rules! deps {
    () => {
        Builder!();
        Logger!();
        Env!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Logger { # [doc = " Creates the logger from the environment."] # [doc = ""] # [doc = " The variables used to read configuration from can be tweaked before"] # [doc = " passing in."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Create a logger reading the log filter from an environment variable"] # [doc = " called `MY_LOG`:"] # [doc = ""] # [doc = " ```"] # [doc = " use env_logger::Logger;"] # [doc = ""] # [doc = " let logger = Logger::from_env(\"MY_LOG\");"] # [doc = " ```"] # [doc = ""] # [doc = " Create a logger using the `MY_LOG` variable for filtering and"] # [doc = " `MY_LOG_STYLE` for whether or not to write styles:"] # [doc = ""] # [doc = " ```"] # [doc = " use env_logger::{Logger, Env};"] # [doc = ""] # [doc = " let env = Env::new().filter_or(\"MY_LOG\", \"info\").write_style_or(\"MY_LOG_STYLE\", \"always\");"] # [doc = ""] # [doc = " let logger = Logger::from_env(env);"] # [doc = " ```"] pub fn from_env < 'a , E > (env : E) -> Self where E : Into < Env < 'a > > , { Builder :: from_env (env) . build () } # [doc = " Creates the logger from the environment using default variable names."] # [doc = ""] # [doc = " This method is a convenient way to call `from_env(Env::default())` without"] # [doc = " having to use the `Env` type explicitly. The logger will use the"] # [doc = " [default environment variables]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Creates a logger using the default environment variables:"] # [doc = ""] # [doc = " ```"] # [doc = " use env_logger::Logger;"] # [doc = ""] # [doc = " let logger = Logger::from_default_env();"] # [doc = " ```"] # [doc = ""] # [doc = " [default environment variables]: struct.Env.html#default-environment-variables"] pub fn from_default_env () -> Self { Builder :: from_default_env () . build () } # [doc = " Returns the maximum `LevelFilter` that this env logger instance is"] # [doc = " configured to output."] pub fn filter (& self) -> LevelFilter { self . filter . filter () } # [doc = " Checks if this record matches the configured filter."] pub fn matches (& self , record : & Record < '_ >) -> bool { self . filter . matches (record) } }
    };
}

impl_6!();