macro_rules! deps {
    () => {
        TargetGround!();
        Gradient!();
    };
}

macro_rules! build_all_gradient_text {
    () => {
        deps!();
        pub fn build_all_gradient_text (text : & str , foreground : Gradient , background : Gradient) -> String { let delta = 1.0 / text . len () as f32 ; let mut result = text . char_indices () . fold (String :: new () , | mut acc , (i , c) | { let step = i as f32 * delta ; let temp = format ! ("\x1B[{};{}m{}" , foreground . at (step) . ansi_color_code (TargetGround :: Foreground) , background . at (step) . ansi_color_code (TargetGround :: Background) , c) ; acc . push_str (& temp) ; acc }) ; result . push_str ("\x1B[0m") ; result }
    };
}

build_all_gradient_text!()