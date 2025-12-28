macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! smallvec {
    () => {
        deps!();
        # [macro_export] macro_rules ! smallvec { (@ one $ x : expr) => (1usize) ; () => ($ crate :: SmallVec :: new ()) ; ($ elem : expr ; $ n : expr) => ({ $ crate :: from_elem ($ elem , $ n) }) ; ($ ($ x : expr) ,+$ (,) ?) => ({ const COUNT : usize = 0usize $ (+ $ crate :: smallvec ! (@ one $ x)) +; let mut vec = $ crate :: SmallVec :: new () ; if COUNT <= vec . capacity () { $ (vec . push ($ x) ;) * vec } else { $ crate :: SmallVec :: from_vec ($ crate :: alloc :: vec ! [$ ($ x ,) +]) } }) ; }
    };
}

smallvec!()