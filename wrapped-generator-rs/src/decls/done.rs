macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! done {
    () => {
        deps!();
        # [doc = " don't use it directly, use done!() macro instead"] # [doc = " would panic if use in none generator context"] # [doc (hidden)] # [inline] pub fn done < T > () -> T { assert ! (is_generator () , "done is only possible in a generator") ; std :: panic :: panic_any (Error :: Done) }
    };
}

done!()