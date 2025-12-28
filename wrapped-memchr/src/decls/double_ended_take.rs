macro_rules! double_ended_take {
    () => {
        # [cfg (not (miri))] pub (crate) fn double_ended_take < I , J > (mut iter : I , take_side : J ,) -> alloc :: vec :: Vec < I :: Item > where I : DoubleEndedIterator , J : Iterator < Item = bool > , { let mut found_front = alloc :: vec ! [] ; let mut found_back = alloc :: vec ! [] ; for take_front in take_side { if take_front { if let Some (pos) = iter . next () { found_front . push (pos) ; } else { break ; } } else { if let Some (pos) = iter . next_back () { found_back . push (pos) ; } else { break ; } } ; } let mut all_found = found_front ; all_found . extend (found_back . into_iter () . rev ()) ; all_found }
    };
}

double_ended_take!();