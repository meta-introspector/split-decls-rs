macro_rules! deps {
    () => {
        Entry!();
        Conflict!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Conflict { # [doc = " Given `entries` and `path_backing`, both values obtained from an [index](gix_index::State), use `start_index` and enumerate"] # [doc = " all conflict stages that still match `entry_path` to produce a conflict description."] # [doc = " Also return the amount of extra-entries that were part of the conflict declaration (not counting the entry at `start_index`)"] # [doc = ""] # [doc = " If for some reason entry at `start_index` isn't in conflicting state, `None` is returned."] # [doc = ""] # [doc = " Return `(Self, num_consumed_entries, three_possibly_entries)`."] pub fn try_from_entry < 'entry > (entries : & 'entry [gix_index :: Entry] , path_backing : & gix_index :: PathStorageRef , start_index : usize , entry_path : & BStr ,) -> Option < (Self , usize , [Option < & 'entry gix_index :: Entry > ; 3]) > { use Conflict :: * ; let mut mask = None :: < u8 > ; let mut seen : [Option < & gix_index :: Entry > ; 3] = Default :: default () ; let mut num_consumed_entries = 0_usize ; for (stage , entry) in (start_index .. (start_index + 3) . min (entries . len ())) . filter_map (| idx | { let entry = & entries [idx] ; let stage = entry . stage_raw () ; (stage > 0 && entry . path_in (path_backing) == entry_path) . then_some ((stage , entry)) }) { * mask . get_or_insert (0) |= match stage { 1 => 0b001 , 2 => 0b010 , 3 => 0b100 , _ => 0 , } ; num_consumed_entries = stage as usize - 1 ; seen [num_consumed_entries] = Some (entry) ; } mask . map (| mask | { (match mask { 0b001 => BothDeleted , 0b010 => AddedByUs , 0b011 => DeletedByThem , 0b100 => AddedByThem , 0b101 => DeletedByUs , 0b110 => BothAdded , 0b111 => BothModified , _ => unreachable ! ("BUG: bitshifts and typical entry layout doesn't allow for more") , } , num_consumed_entries , seen ,) }) } }
    };
}

impl_27!();