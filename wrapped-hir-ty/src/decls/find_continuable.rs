macro_rules! deps {
    () => {
        BreakableKind!();
        BreakableContext!();
    };
}

macro_rules! find_continuable {
    () => {
        deps!();
        fn find_continuable < 'a , 'db > (ctxs : & 'a mut [BreakableContext < 'db >] , label : Option < LabelId > ,) -> Option < & 'a mut BreakableContext < 'db > > { match label { Some (_) => find_breakable (ctxs , label) . filter (| it | matches ! (it . kind , BreakableKind :: Loop)) , None => find_breakable (ctxs , label) , } }
    };
}

find_continuable!()