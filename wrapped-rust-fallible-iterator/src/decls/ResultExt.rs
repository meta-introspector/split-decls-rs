macro_rules! ResultExt {
    () => {
        trait ResultExt < T , E > { fn unpack_fold (self) -> Result < T , E > ; }
    };
}

ResultExt!()