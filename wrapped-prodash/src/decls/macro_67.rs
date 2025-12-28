macro_rules! macro_67 {
    () => {
        # [cfg (all (feature = "render-line" , not (any (feature = "render-line-crossterm"))))] compile_error ! ("Please use the 'render-line-crossterm' feature") ;
    };
}

macro_67!();