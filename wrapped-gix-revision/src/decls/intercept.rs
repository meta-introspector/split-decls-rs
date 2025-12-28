macro_rules! deps {
    () => {
        Traversal!();
        Kind!();
        Navigate!();
        ReflogLookup!();
        PrefixHint!();
        Delegate!();
        SiblingBranch!();
        Revision!();
        PeelTo!();
    };
}

macro_rules! intercept {
    () => {
        deps!();
        mod intercept { use bstr :: { BStr , BString } ; use crate :: spec :: parse :: { delegate , Delegate } ; # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub (crate) enum PrefixHintOwned { MustBeCommit , DescribeAnchor { ref_name : BString , generation : usize } , } impl PrefixHintOwned { pub fn to_ref (& self) -> delegate :: PrefixHint < '_ > { match self { PrefixHintOwned :: MustBeCommit => delegate :: PrefixHint :: MustBeCommit , PrefixHintOwned :: DescribeAnchor { ref_name , generation } => delegate :: PrefixHint :: DescribeAnchor { ref_name : ref_name . as_ref () , generation : * generation , } , } } } impl < 'a > From < delegate :: PrefixHint < 'a > > for PrefixHintOwned { fn from (v : delegate :: PrefixHint < 'a >) -> Self { match v { delegate :: PrefixHint :: MustBeCommit => PrefixHintOwned :: MustBeCommit , delegate :: PrefixHint :: DescribeAnchor { generation , ref_name } => PrefixHintOwned :: DescribeAnchor { ref_name : ref_name . to_owned () , generation , } , } } } pub (crate) struct InterceptRev < 'a , T > { pub inner : & 'a mut T , pub last_ref : Option < BString > , pub last_prefix : Option < (gix_hash :: Prefix , Option < PrefixHintOwned >) > , pub done : bool , } impl < 'a , T > InterceptRev < 'a , T > where T : Delegate , { pub fn new (delegate : & 'a mut T) -> Self { InterceptRev { inner : delegate , last_ref : None , last_prefix : None , done : false , } } } impl < T > Delegate for InterceptRev < '_ , T > where T : Delegate , { fn done (& mut self) { self . done = true ; self . inner . done () ; } } impl < T > delegate :: Revision for InterceptRev < '_ , T > where T : Delegate , { fn find_ref (& mut self , name : & BStr) -> Option < () > { self . last_ref = name . to_owned () . into () ; self . inner . find_ref (name) } fn disambiguate_prefix (& mut self , prefix : gix_hash :: Prefix , hint : Option < delegate :: PrefixHint < '_ > > ,) -> Option < () > { self . last_prefix = Some ((prefix , hint . map (Into :: into))) ; self . inner . disambiguate_prefix (prefix , hint) } fn reflog (& mut self , query : delegate :: ReflogLookup) -> Option < () > { self . inner . reflog (query) } fn nth_checked_out_branch (& mut self , branch_no : usize) -> Option < () > { self . inner . nth_checked_out_branch (branch_no) } fn sibling_branch (& mut self , kind : delegate :: SiblingBranch) -> Option < () > { self . inner . sibling_branch (kind) } } impl < T > delegate :: Navigate for InterceptRev < '_ , T > where T : Delegate , { fn traverse (& mut self , kind : delegate :: Traversal) -> Option < () > { self . inner . traverse (kind) } fn peel_until (& mut self , kind : delegate :: PeelTo < '_ >) -> Option < () > { self . inner . peel_until (kind) } fn find (& mut self , regex : & BStr , negated : bool) -> Option < () > { self . inner . find (regex , negated) } fn index_lookup (& mut self , path : & BStr , stage : u8) -> Option < () > { self . inner . index_lookup (path , stage) } } impl < T > delegate :: Kind for InterceptRev < '_ , T > where T : Delegate , { fn kind (& mut self , kind : crate :: spec :: Kind) -> Option < () > { self . inner . kind (kind) } } }
    };
}

intercept!();