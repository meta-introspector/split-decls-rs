macro_rules! preserve_objects_for_their_debuginfo {
    () => {
        # [doc = " Returns a pair of boolean indicating whether we should preserve the object and"] # [doc = " dwarf object files on the filesystem for their debug information. This is often"] # [doc = " useful with split-dwarf like schemes."] fn preserve_objects_for_their_debuginfo (sess : & Session) -> (bool , bool) { if sess . opts . debuginfo == config :: DebugInfo :: None { return (false , false) ; } match (sess . split_debuginfo () , sess . opts . unstable_opts . split_dwarf_kind) { (SplitDebuginfo :: Off , _) => (false , false) , (SplitDebuginfo :: Packed , _) => (false , false) , (SplitDebuginfo :: Unpacked , _) if ! sess . target_can_use_split_dwarf () => (true , false) , (SplitDebuginfo :: Unpacked , SplitDwarfKind :: Single) => (true , false) , (SplitDebuginfo :: Unpacked , SplitDwarfKind :: Split) => (false , true) , } }
    };
}

preserve_objects_for_their_debuginfo!();