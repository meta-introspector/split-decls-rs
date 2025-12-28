macro_rules! deps {
    () => {
        RunnableData!();
        Runnable!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl From < RunnableData > for Runnable { fn from (data : RunnableData) -> Self { Runnable { program : data . program , args : data . args , cwd : data . cwd , kind : data . kind . into () } } }
    };
}

impl_20!();