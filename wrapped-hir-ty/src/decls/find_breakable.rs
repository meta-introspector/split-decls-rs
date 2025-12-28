macro_rules! deps {
    () => {
        BreakableKind!();
        BreakableContext!();
    };
}

macro_rules! find_breakable {
    () => {
        deps!();
        fn find_breakable < 'a , 'db > (ctxs : & 'a mut [BreakableContext < 'db >] , label : Option < LabelId > ,) -> Option < & 'a mut BreakableContext < 'db > > { let mut ctxs = ctxs . iter_mut () . rev () . take_while (| it | matches ! (it . kind , BreakableKind :: Block | BreakableKind :: Loop)) ; match label { Some (_) => ctxs . find (| ctx | ctx . label == label) , None => ctxs . find (| ctx | matches ! (ctx . kind , BreakableKind :: Loop)) , } }
    };
}

find_breakable!()