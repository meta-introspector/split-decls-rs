macro_rules! SuffixOrdering {
    () => {
        # [doc = " The result of comparing corresponding bytes between two suffixes."] # [derive (Clone , Copy , Debug)] enum SuffixOrdering { # [doc = " This occurs when the given candidate byte indicates that the candidate"] # [doc = " suffix is better than the current maximal (or minimal) suffix. That is,"] # [doc = " the current candidate suffix should supplant the current maximal (or"] # [doc = " minimal) suffix."] Accept , # [doc = " This occurs when the given candidate byte excludes the candidate suffix"] # [doc = " from being better than the current maximal (or minimal) suffix. That"] # [doc = " is, the current candidate suffix should be dropped and the next one"] # [doc = " should be considered."] Skip , # [doc = " This occurs when no decision to accept or skip the candidate suffix"] # [doc = " can be made, e.g., when corresponding bytes are equivalent. In this"] # [doc = " case, the next corresponding bytes should be compared."] Push , }
    };
}

SuffixOrdering!();