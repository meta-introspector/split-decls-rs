macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! EnvGetter {
    () => {
        deps!();
        pub trait EnvGetter { fn get_env (& self , name : & 'static str) -> Option < Env > ; }
    };
}

EnvGetter!()