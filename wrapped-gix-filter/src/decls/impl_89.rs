macro_rules! deps {
    () => {
        State!();
        Context!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [doc = " Initialization"] impl State { # [doc = " Create a new instance using `context` to inform launched processes about their environment."] pub fn new (context : gix_command :: Context) -> Self { Self { running : Default :: default () , context , } } }
    };
}

impl_89!()