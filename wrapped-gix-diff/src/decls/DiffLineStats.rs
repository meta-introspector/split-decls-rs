macro_rules! DiffLineStats {
    () => {
        # [doc = " Information about the diff performed to detect similarity."] # [derive (Debug , Default , Clone , Copy , PartialEq , PartialOrd)] pub struct DiffLineStats { # [doc = " The amount of lines to remove from the source to get to the destination."] pub removals : u32 , # [doc = " The amount of lines to add to the source to get to the destination."] pub insertions : u32 , # [doc = " The amount of lines of the previous state, in the source."] pub before : u32 , # [doc = " The amount of lines of the new state, in the destination."] pub after : u32 , # [doc = " A range from 0 to 1.0, where 1.0 is a perfect match and 0.5 is a similarity of 50%."] # [doc = " Similarity is the ratio between all lines in the previous blob and the current blob,"] # [doc = " calculated as `(old_lines_count - new_lines_count) as f32 / old_lines_count.max(new_lines_count) as f32`."] pub similarity : f32 , }
    };
}

DiffLineStats!()