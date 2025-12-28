macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_post_link_objects {
    () => {
        deps!();
        # [doc = " Add post-link object files defined by the target spec."] fn add_post_link_objects (cmd : & mut dyn Linker , sess : & Session , link_output_kind : LinkOutputKind , self_contained : bool ,) { let objects = if self_contained { & sess . target . post_link_objects_self_contained } else { & sess . target . post_link_objects } ; for obj in objects . get (& link_output_kind) . iter () . copied () . flatten () { cmd . add_object (& get_object_file_path (sess , obj , self_contained)) ; } }
    };
}

add_post_link_objects!()