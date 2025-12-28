macro_rules! deps {
    () => {
        HirWrite!();
        HirDatabase!();
        DisplayLifetime!();
        ClosureStyle!();
        DisplayKind!();
        DisplayTarget!();
        BoundsFormattingCtx!();
    };
}

macro_rules! HirFormatter {
    () => {
        deps!();
        pub struct HirFormatter < 'a , 'db > { # [doc = " The database handle"] pub db : & 'db dyn HirDatabase , pub interner : DbInterner < 'db > , # [doc = " The sink to write into"] fmt : & 'a mut dyn HirWrite , # [doc = " A buffer to intercept writes with, this allows us to track the overall size of the formatted output."] buf : String , # [doc = " The current size of the formatted output."] curr_size : usize , # [doc = " Size from which we should truncate the output."] max_size : Option < usize > , # [doc = " When rendering something that has a concept of \"children\" (like fields in a struct), this limits"] # [doc = " how many should be rendered."] pub entity_limit : Option < usize > , # [doc = " When rendering functions, whether to show the constraint from the container"] show_container_bounds : bool , omit_verbose_types : bool , closure_style : ClosureStyle , display_lifetimes : DisplayLifetime , display_kind : DisplayKind , display_target : DisplayTarget , bounds_formatting_ctx : BoundsFormattingCtx < 'db > , }
    };
}

HirFormatter!()