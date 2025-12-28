macro_rules! shuffle {
    () => {
        macro_rules ! shuffle { ($ a : expr , $ b : expr , $ z : expr , $ y : expr , $ x : expr , $ w : expr) => { i32x4_shuffle ::< { $ w } , { $ x } , { $ y + 4 } , { $ z + 4 } > ($ a , $ b) } ; }
    };
}

shuffle!();