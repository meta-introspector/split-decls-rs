macro_rules! deps {
    () => {
        Id!();
        Result!();
        AnyValueId!();
        MatchedArg!();
        SubCommand!();
        ArgMatches!();
        MatchesError!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl ArgMatches { # [inline] fn try_get_arg (& self , arg : & str) -> Result < Option < & MatchedArg > , MatchesError > { ok ! (self . verify_arg (arg)) ; Ok (self . args . get (arg)) } # [inline] fn try_get_arg_t < T : Any + Send + Sync + 'static > (& self , arg : & str ,) -> Result < Option < & MatchedArg > , MatchesError > { let arg = match ok ! (self . try_get_arg (arg)) { Some (arg) => arg , None => { return Ok (None) ; } } ; ok ! (self . verify_arg_t ::< T > (arg)) ; Ok (Some (arg)) } # [inline] fn try_remove_arg_t < T : Any + Send + Sync + 'static > (& mut self , arg : & str ,) -> Result < Option < MatchedArg > , MatchesError > { ok ! (self . verify_arg (arg)) ; let (id , matched) = match self . args . remove_entry (arg) { Some ((id , matched)) => (id , matched) , None => { return Ok (None) ; } } ; let expected = AnyValueId :: of :: < T > () ; let actual = matched . infer_type_id (expected) ; if actual == expected { Ok (Some (matched)) } else { self . args . insert (id , matched) ; Err (MatchesError :: Downcast { actual , expected }) } } fn verify_arg_t < T : Any + Send + Sync + 'static > (& self , arg : & MatchedArg ,) -> Result < () , MatchesError > { let expected = AnyValueId :: of :: < T > () ; let actual = arg . infer_type_id (expected) ; if expected == actual { Ok (()) } else { Err (MatchesError :: Downcast { actual , expected }) } } # [inline] fn verify_arg (& self , _arg : & str) -> Result < () , MatchesError > { # [cfg (debug_assertions)] { if _arg == Id :: EXTERNAL || self . valid_args . iter () . any (| s | * s == _arg) { } else { debug ! ("`{:?}` is not an id of an argument or a group.\n\
                     Make sure you're using the name of the argument itself \
                     and not the name of short or long flags." , _arg) ; return Err (MatchesError :: UnknownArgument { }) ; } } Ok (()) } # [inline] # [cfg_attr (debug_assertions , track_caller)] fn get_arg < 's > (& 's self , arg : & str) -> Option < & 's MatchedArg > { # [cfg (debug_assertions)] { if arg == Id :: EXTERNAL || self . valid_args . iter () . any (| s | * s == arg) { } else { panic ! ("`{arg:?}` is not an id of an argument or a group.\n\
                     Make sure you're using the name of the argument itself \
                     and not the name of short or long flags.") ; } } self . args . get (arg) } # [inline] # [cfg_attr (debug_assertions , track_caller)] fn get_subcommand (& self , name : & str) -> Option < & SubCommand > { # [cfg (debug_assertions)] { if name . is_empty () || self . valid_subcommands . iter () . any (| s | * s == name) { } else { panic ! ("`{name}` is not a name of a subcommand.") ; } } if let Some (ref sc) = self . subcommand { if sc . name == name { return Some (sc) ; } } None } }
    };
}

impl_442!();