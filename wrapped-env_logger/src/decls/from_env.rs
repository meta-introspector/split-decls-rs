macro_rules! deps {
    () => {
        Builder!();
        Env!();
    };
}

macro_rules! from_env {
    () => {
        deps!();
        # [doc = " Create a builder from the given environment variables."] # [doc = ""] # [doc = " The builder can be configured before being initialized."] # [deprecated (since = "0.8.0" , note = "Prefer `env_logger::Builder::from_env()` instead.")] pub fn from_env < 'a , E > (env : E) -> Builder where E : Into < Env < 'a > > , { Builder :: from_env (env) }
    };
}

from_env!()