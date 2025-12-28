macro_rules! ok_if_any {
    () => {
        # [doc = " This is similar to `collect::<Result<Vec<_>, _>>`, but unlike it, it succeeds if there is *any* `Ok` item."] fn ok_if_any < T , E > (iter : impl Iterator < Item = Result < T , E > >) -> Result < Vec < T > , E > { let mut err = None ; let oks = iter . filter_map (| item | match item { Ok (it) => Some (it) , Err (it) => { err = Some (it) ; None } }) . collect :: < Vec < _ > > () ; if ! oks . is_empty () { Ok (oks) } else if let Some (err) = err { Err (err) } else { Ok (Vec :: new ()) } }
    };
}

ok_if_any!()