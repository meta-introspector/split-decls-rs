macro_rules! convert_size {
    () => {
        fn convert_size (size : Option < (usize , usize) >) -> Option < (u32 , u32) > { if let Some ((w , h)) = size { return Some ((w as u32 , h as u32)) ; } None }
    };
}

convert_size!()