macro_rules! macro_58 {
    () => {
        # [cfg (not (any (feature = "render-tui-crossterm")))] compile_error ! ("Please set the 'render-tui-crossterm' feature when using the 'render-tui'") ;
    };
}

macro_58!()