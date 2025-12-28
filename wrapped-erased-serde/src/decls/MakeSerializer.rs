macro_rules! MakeSerializer {
    () => {
        struct MakeSerializer < TraitObject > (TraitObject) ;
    };
}

MakeSerializer!()