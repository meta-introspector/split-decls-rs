macro_rules! image {
    () => {
        # [cfg (feature = "image")] # [cfg (all (target_arch = "x86_64" , target_pointer_width = "32"))] # [path = "x32/image.rs"] pub mod image ;
    };
}

image!()