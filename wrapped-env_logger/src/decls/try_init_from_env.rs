macro_rules! deps {
    () => {
        Builder!();
        Env!();
    };
}

macro_rules! try_init_from_env {
    () => {
        deps!();
        # [doc = " Attempts to initialize the global logger with an env logger from the given"] # [doc = " environment variables."] # [doc = ""] # [doc = " This should be called early in the execution of a Rust program. Any log"] # [doc = " events that occur before initialization will be ignored."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Initialise a logger using the `MY_LOG` environment variable for filters"] # [doc = " and `MY_LOG_STYLE` for writing colors:"] # [doc = ""] # [doc = " ```"] # [doc = " use env_logger::{Builder, Env};"] # [doc = ""] # [doc = " # fn run() -> Result<(), Box<dyn ::std::error::Error>> {"] # [doc = " let env = Env::new().filter(\"MY_LOG\").write_style(\"MY_LOG_STYLE\");"] # [doc = ""] # [doc = " env_logger::try_init_from_env(env)?;"] # [doc = ""] # [doc = " Ok(())"] # [doc = " # }"] # [doc = " # run().unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will fail if it is called more than once, or if another"] # [doc = " library has already initialized a global logger."] pub fn try_init_from_env < 'a , E > (env : E) -> Result < () , SetLoggerError > where E : Into < Env < 'a > > , { let mut builder = Builder :: from_env (env) ; builder . try_init () }
    };
}

try_init_from_env!();