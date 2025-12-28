macro_rules! deps {
    () => {
        Delegate!();
        Error!();
        Spec!();
        Kind!();
        Merge!();
        Repository!();
        Default!();
        Options!();
    };
}

macro_rules! impl_882 {
    () => {
        deps!();
        impl < 'repo > Delegate < 'repo > { pub fn new (repo : & 'repo Repository , opts : crate :: revision :: spec :: parse :: Options) -> Self { Delegate { refs : Default :: default () , objs : Default :: default () , paths : Default :: default () , ambiguous_objects : Default :: default () , idx : 0 , kind : None , err : Vec :: new () , prefix : Default :: default () , last_call_was_disambiguate_prefix : Default :: default () , opts , repo , } } pub fn into_err (mut self) -> Error { let repo = self . repo ; for err in self . ambiguous_objects . iter_mut () . zip (self . prefix) . filter_map (| (a , b) | a . take () . filter (| candidates | candidates . len () > 1) . zip (b)) . map (| (candidates , prefix) | Error :: ambiguous (candidates , prefix , repo)) . rev () { self . err . insert (0 , err) ; } Error :: from_errors (self . err) } pub fn into_rev_spec (mut self) -> Result < crate :: revision :: Spec < 'repo > , Error > { fn zero_or_one_objects_or_ambiguity_err (mut candidates : [Option < HashSet < ObjectId > > ; 2] , prefix : [Option < gix_hash :: Prefix > ; 2] , mut errors : Vec < Error > , repo : & Repository ,) -> Result < [Option < ObjectId > ; 2] , Error > { let mut out = [None , None] ; for ((candidates , prefix) , out) in candidates . iter_mut () . zip (prefix) . zip (out . iter_mut ()) { let candidates = candidates . take () ; match candidates { None => * out = None , Some (candidates) => match candidates . len () { 0 => { unreachable ! ("BUG: let's avoid still being around if no candidate matched the requirements") } 1 => { * out = candidates . into_iter () . next () ; } _ => { errors . insert (0 , Error :: ambiguous (candidates , prefix . expect ("set when obtaining candidates") , repo) ,) ; return Err (Error :: from_errors (errors)) ; } } , } } Ok (out) } fn kind_to_spec (kind : Option < gix_revision :: spec :: Kind > , [first , second] : [Option < ObjectId > ; 2] ,) -> Result < gix_revision :: Spec , Error > { use gix_revision :: spec :: Kind :: * ; Ok (match kind . unwrap_or_default () { IncludeReachable => gix_revision :: Spec :: Include (first . ok_or (Error :: Malformed) ?) , ExcludeReachable => gix_revision :: Spec :: Exclude (first . ok_or (Error :: Malformed) ?) , RangeBetween => gix_revision :: Spec :: Range { from : first . ok_or (Error :: Malformed) ? , to : second . ok_or (Error :: Malformed) ? , } , ReachableToMergeBase => gix_revision :: Spec :: Merge { theirs : first . ok_or (Error :: Malformed) ? , ours : second . ok_or (Error :: Malformed) ? , } , IncludeReachableFromParents => gix_revision :: Spec :: IncludeOnlyParents (first . ok_or (Error :: Malformed) ?) , ExcludeReachableFromParents => gix_revision :: Spec :: ExcludeParents (first . ok_or (Error :: Malformed) ?) , }) } let range = zero_or_one_objects_or_ambiguity_err (self . objs , self . prefix , self . err , self . repo) ? ; Ok (crate :: revision :: Spec { path : self . paths [0] . take () . or (self . paths [1] . take ()) , first_ref : self . refs [0] . take () , second_ref : self . refs [1] . take () , inner : kind_to_spec (self . kind , range) ? , repo : self . repo , }) } }
    };
}

impl_882!();