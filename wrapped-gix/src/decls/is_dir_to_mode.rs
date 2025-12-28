macro_rules! is_dir_to_mode {
    () => {
        # [cfg (feature = "attributes")] fn is_dir_to_mode (is_dir : bool) -> gix_index :: entry :: Mode { if is_dir { gix_index :: entry :: Mode :: DIR } else { gix_index :: entry :: Mode :: FILE } }
    };
}

is_dir_to_mode!()