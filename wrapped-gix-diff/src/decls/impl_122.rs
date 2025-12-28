macro_rules! deps {
    () => {
        Pipeline!();
        Platform!();
        Mode!();
        Options!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Platform { # [doc = " Create a new instance with `options`, and a way to `filter` data from the object database to data that is diff-able."] # [doc = " `filter_mode` decides how to do that specifically."] # [doc = " Use `attr_stack` to access attributes pertaining worktree filters and diff settings."] pub fn new (options : Options , filter : Pipeline , filter_mode : pipeline :: Mode , attr_stack : gix_worktree :: Stack ,) -> Self { Platform { old : None , new : None , diff_cache : Default :: default () , free_list : Vec :: with_capacity (2) , options , filter , filter_mode , attr_stack , } } }
    };
}

impl_122!()