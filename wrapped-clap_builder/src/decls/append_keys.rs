macro_rules! deps {
    () => {
        KeyType!();
        Arg!();
        Key!();
    };
}

macro_rules! append_keys {
    () => {
        deps!();
        # [doc = " Generate key types for an specific Arg."] fn append_keys (keys : & mut Vec < Key > , arg : & Arg , index : usize) { if let Some (pos_index) = arg . index { let key = KeyType :: Position (pos_index) ; keys . push (Key { key , index }) ; } else { if let Some (short) = arg . short { let key = KeyType :: Short (short) ; keys . push (Key { key , index }) ; } if let Some (long) = arg . long . clone () { let key = KeyType :: Long (long . into ()) ; keys . push (Key { key , index }) ; } for (short , _) in arg . short_aliases . iter () { let key = KeyType :: Short (* short) ; keys . push (Key { key , index }) ; } for (long , _) in arg . aliases . iter () { let key = KeyType :: Long (long . into ()) ; keys . push (Key { key , index }) ; } } }
    };
}

append_keys!();