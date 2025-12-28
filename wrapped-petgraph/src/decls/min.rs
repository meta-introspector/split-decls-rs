macro_rules! min {
    () => {
        # [doc = " Returns the minimum value between given `a` and `b`."] # [doc = " Will panic if it tries to compare two elements that aren't comparable"] # [doc = " (i.e., given two elements `a` and `b`, neither `a >= b` nor `a < b`)."] fn min < G > (a : G :: EdgeWeight , b : G :: EdgeWeight) -> G :: EdgeWeight where G : Data , G :: EdgeWeight : PartialOrd , { if a < b { a } else if a >= b { b } else { panic ! ("Invalid edge weights. Impossible to get min value.") ; } }
    };
}

min!();