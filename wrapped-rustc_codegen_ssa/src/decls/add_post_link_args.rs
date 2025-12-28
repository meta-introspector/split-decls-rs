macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_post_link_args {
    () => {
        deps!();
        # [doc = " Add arbitrary \"post-link\" args defined by the target spec."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_post_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if let Some (args) = sess . target . post_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } }
    };
}

add_post_link_args!()