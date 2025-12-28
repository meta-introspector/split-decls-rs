macro_rules! Bitmaps {
    () => {
        # [doc = " Bitmaps to know which entries to delete or replace, even though details are still unknown."] # [derive (Clone)] pub struct Bitmaps { # [doc = " A bitmap to signal which entries to delete, maybe."] pub delete : gix_bitmap :: ewah :: Vec , # [doc = " A bitmap to signal which entries to replace, maybe."] pub replace : gix_bitmap :: ewah :: Vec , }
    };
}

Bitmaps!()