macro_rules! PREDEFINED_TOOLS {
    () => {
        const PREDEFINED_TOOLS : & [SmolStr] = & [SmolStr :: new_static ("clippy") , SmolStr :: new_static ("rustfmt") , SmolStr :: new_static ("diagnostic") , SmolStr :: new_static ("miri") , SmolStr :: new_static ("rust_analyzer") ,] ;
    };
}

PREDEFINED_TOOLS!()