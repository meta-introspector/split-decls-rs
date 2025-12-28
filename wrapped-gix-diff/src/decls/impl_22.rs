macro_rules! deps {
    () => {
        Rewrites!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [doc = " The default settings for rewrites according to the git configuration defaults."] impl Default for Rewrites { fn default () -> Self { Rewrites { copies : None , percentage : Some (0.5) , limit : 1000 , track_empty : false , } } }
    };
}

impl_22!()