macro_rules! deps {
    () => {
        Options!();
        Driver!();
        Mode!();
        Platform!();
        Outcome!();
        Pipeline!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Platform { # [doc = " Create a new instance with a way to `filter` data from the object database and turn it into something that is merge-able."] # [doc = " `filter_mode` decides how to do that specifically."] # [doc = " Use `attr_stack` to access attributes pertaining worktree filters and merge settings."] # [doc = " `drivers` are the list of available merge drivers that individual paths can refer to by means of git attributes."] # [doc = " `options` further configure the operation."] pub fn new (filter : Pipeline , filter_mode : pipeline :: Mode , attr_stack : gix_worktree :: Stack , mut drivers : Vec < super :: Driver > , options : Options ,) -> Self { drivers . sort_by (| a , b | a . name . cmp (& b . name)) ; Platform { drivers , current : None , ancestor : None , other : None , filter , filter_mode , attr_stack , attrs : { let mut out = attributes :: search :: Outcome :: default () ; out . initialize_with_selection (& Default :: default () , ["merge" , "conflict-marker-size"]) ; out } , options , } } }
    };
}

impl_59!()