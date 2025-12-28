macro_rules! deps {
    () => {
        Indel!();
    };
}

macro_rules! coalesce_indels {
    () => {
        deps!();
        fn coalesce_indels (indels : Vec < Indel >) -> Vec < Indel > { indels . into_iter () . coalesce (| mut a , b | { if a . delete . end () == b . delete . start () { a . insert . push_str (& b . insert) ; a . delete = TextRange :: new (a . delete . start () , b . delete . end ()) ; Ok (a) } else { Err ((a , b)) } }) . collect_vec () }
    };
}

coalesce_indels!()